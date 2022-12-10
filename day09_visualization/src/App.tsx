import { Component, createEffect, createSignal, For, Index, onMount, Show } from 'solid-js';
// import produce from "immer";
import { createStore, produce } from 'solid-js/store';
import { TransitionGroup } from "solid-transition-group";

import rope from './assets/rope.png';
import knot from './assets/knot.png';
import knotEnd from './assets/knot_end.png';
import styles from './App.module.css';

type KnotType = "middle" | "end";
type Position = { x: number, y: number };
type KnotsModel = {
  index: number,
  position: Position,
  knotType: KnotType,
  knotRotation: number,
  showRope: boolean,
  ropeLength?: number,
  ropeRotation?: number
};

const start_knots = [
  { x: 2, y: 0 },
  { x: 2, y: 1 },
  { x: 2, y: 2 },
  { x: 1, y: 3 },
];

const App: Component = () => {
  const [knots, setKnots] = createSignal(start_knots);
  const [knotsModel, setKnotsModel] = createStore<Array<KnotsModel>>([]);

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

  const reset = () => {
    setKnots([]);
    setKnots(start_knots);
  }

  const moveLeft = () => setKnots(prev => [{ x: prev[0].x - 1, y: prev[0].y }, ...prev.slice(1, 4)]);
  const moveMiddleRight = () => setKnots(prev => [...prev.slice(0, 2), { x: prev[2].x + 1, y: prev[2].y }, prev[3]]);
  const moveRight = () => setKnots(prev => [...prev.slice(0, 3), { x: prev[3].x + 1, y: prev[3].y }]);

  return (
    <div class={styles.App}>
      <button onClick={reset}>Reset</button>
      <button onClick={moveLeft}>Left</button>
      <button onClick={moveMiddleRight}>Middle Right</button>
      <button onClick={moveRight}>Right</button>
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
    </div>
  );
};

const Knot: Component<{ position: Position, type: KnotType, rotation: number}> = (props) => {
  onMount(() => {
    console.log("mounting knot");
  });
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

