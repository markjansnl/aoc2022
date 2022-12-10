import { Component, createEffect, createSignal, For, Index, onMount, Show } from 'solid-js';
import { createStore, produce } from 'solid-js/store';
import init, { RopeModel } from "wasm";

import rope from './assets/rope.png';
import knot from './assets/knot.png';
import knotEnd from './assets/knot_end.png';
import styles from './App.module.css';

type KnotType = "middle" | "end";
type Position = { x: number, y: number };
type RopeModelType = RopeModel | undefined;
type KnotsModel = {
  index: number,
  position: Position,
  knotType: KnotType,
  knotRotation: number,
  showRope: boolean,
  ropeLength?: number,
  ropeRotation?: number
};

const start_knots = new Array(10).fill({ x: 0, y: 0 });

const App: Component = () => {
  const [wasmLoaded, setWasmLoaded] = createSignal(false);
  const [knots, setKnots] = createSignal(start_knots);
  const [knotsModel, setKnotsModel] = createStore<Array<KnotsModel>>([]);

  let ropeModel = undefined as RopeModelType;
  init().then(() => {
    ropeModel = new RopeModel();
    setWasmLoaded(true);
  });

  createEffect(() => setKnotsModel(produce(model => {
    while (knots().length > model.length) {
      // @ts-ignore
      model.push({});
    }
    if (knots.length < model.length) {
      model.splice(knots().length);
    }
    model.forEach((knot, index) => {
      const prev = knots()[index - 1] || { x: 0, y: 0 };
      const next = knots()[index + 1] || { x: 0, y: 0 };

      knot.index = index;
      knot.position = knots()[index];
      knot.knotType = index == 0 || index == start_knots.length - 1 ? "end" : "middle" as KnotType;
      knot.knotRotation = index === 0
        ? Math.PI + Math.atan2((knot.position.x - next.x), (next.y - knot.position.y))
        : Math.atan2((prev.x - knot.position.x), (knot.position.y - prev.y));
      knot.showRope = index < start_knots.length - 1;
      if (knot.showRope) {
        knot.ropeLength = Math.sqrt(Math.pow(knot.position.x - next.x, 2) + Math.pow(knot.position.y - next.y, 2));
        knot.ropeRotation = Math.atan2((knot.position.x - next.x), (next.y - knot.position.y));
      }
    });
  })));

  // const reset = () => {
  //   setKnots([]);
  //   setKnots(start_knots);
  // }

  const start = () => setInterval(step, 100);

  const step = () => {
    const knot_result = ropeModel?.step();
    if (knot_result) {
      setKnots(prev => [...prev.slice(0, knot_result.index), knot_result.position, ...prev.slice(knot_result.index + 1, prev.length)]);
    }
  }

  return (
    <div class={styles.App}>
      <Show when={wasmLoaded()} fallback={<h3>Loading wasm module...</h3>}>
      {/* <button onClick={reset} style={{ "margin-top": "1rem" }}>Reset</button> */}
      <button onClick={start} style={{ "margin": "1rem" }}>Start</button>
        <div class={styles.Grid}>
          <For each={knotsModel}>{knot =>
            <>
              <Knot position={knot.position} type={knot.knotType} rotation={knot.knotRotation} />
              <Show when={ knot.showRope }>
                <Rope position={knot.position} rotation={knot.ropeRotation} length={knot.ropeLength} />
              </Show>
            </>
          }</For>
        </div>
      </Show>
    </div>
  );
};

const Knot: Component<{ position: Position, type: KnotType, rotation: number}> = (props) => {
  return (
    <img
      src={ props.type === "middle" ? knot : knotEnd }
      class={ props.type === "middle" ? styles.Knot : styles.KnotEnd }
      style={{
        rotate: `${props.rotation}rad`,
        left: `${props.position.x * 2}rem`,
        top: `${props.position.y * 2}rem` 
      }}
      alt="knot" />
  );
};

const Rope: Component<{rotation: number | undefined, position: Position, length: number | undefined }> = (props) => {
  return (
    <img
      src={rope}
      class={styles.Rope}
      style={{
        rotate: `${props.rotation}rad`,
        left: `${props.position.x * 2}rem`,
        top: `${props.position.y * 2}rem`,
        height: `${(props.length || 0) * 2}rem`
      }} alt="rope" />
  );
};

export default App;

