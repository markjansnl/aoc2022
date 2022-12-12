import { Component, createSignal, For, Match, Show, Switch } from 'solid-js';
import { createStore, produce } from 'solid-js/store';
import init, { KeepAway, example } from 'wasm';

import catcher from './assets/catcher.png';
import monkey from './assets/monkey.png';
import styles from './App.module.css';

const SMALL_DELAY = 200;
const THROW_DELAY = 500;

type Model = {
  round: number,
  catcher_flipped: boolean,
  monkies: Monkey[],
  items: Item[],
};

type Monkey = {
  pos: number,
  items: number[],
  itemsInspected: number
};

type Item = {
  pos: number,
  index: number,
  worryLevel: number
};

const delay = (d: number) => new Promise(resolve => setTimeout(resolve, d));

const App: Component = () => {
  const [wasmLoaded, setWasmLoaded] = createSignal(false);
  const [showInput, setShowInput] = createSignal(true);
  const [input, setInput] = createSignal("");
  const [model, setModel] = createStore<Model>({
    round: 1,
    catcher_flipped: false,
    monkies: [],
    items: [],
  });

  let keepAway = undefined as unknown as KeepAway;
  init().then(() => {
    setWasmLoaded(true);
    setInput(example());
  });

  const onStart = () => {
    setShowInput(false);
    keepAway = new KeepAway(input());
    const items: Array<Item> = [];
    const monkies: Array<Monkey> = Array.from(new Array(keepAway.monkey_count())).map((_, i, a) => {
      const monkeyItems = !keepAway ? [] : Array.from(keepAway.monkey_items(i)).map((worryLevel, j) => {
        items.push({
          pos: i / a.length,
          index: j + 1,
          worryLevel
        })
        return items.length - 1;
      });

      return {
        pos: i / a.length,
        items: monkeyItems,
        itemsInspected: 0
      };
    });

    setModel({
      catcher_flipped: false,
      monkies,
      items
    });

    (async () => {
      for (let round = 0; round < 20; round++) {
        for (let monkeyIndex = 0; monkeyIndex < model.monkies.length; monkeyIndex++) {
          setModel(produce(draft => {
            draft.round = round + 1;
            draft.catcher_flipped = draft.monkies[monkeyIndex].pos > 0.25 && draft.monkies[monkeyIndex].pos <= 0.75;
          }));

          const itemCount = model.monkies[monkeyIndex].items.length;
          for (let itemIndex = 0; itemIndex < itemCount; itemIndex++) {
            setModel(produce(draft => {
              for (let i = 0; i < model.monkies[monkeyIndex].items.length; i++) {
                draft.items[model.monkies[monkeyIndex].items[i]].index = i;
              }
            }));
            await delay(SMALL_DELAY);

            setModel(produce(draft => {
              draft.items[model.monkies[monkeyIndex].items[0]].worryLevel = keepAway.inspect();
              draft.monkies[monkeyIndex].itemsInspected++;
            }));
            await delay(SMALL_DELAY);

            setModel(produce(draft => draft.items[model.monkies[monkeyIndex].items[0]].worryLevel = keepAway.get_borred()));
            await delay(SMALL_DELAY);

            setModel(produce(draft => {
              const throwTo = keepAway.throw();
              const index = draft.monkies[monkeyIndex].items.splice(0, 1)[0];
              draft.monkies[throwTo].items.push(index);
              draft.items[index].pos = model.monkies[throwTo].pos;
              draft.items[index].index = model.monkies[throwTo].items.length;
            }));
            await delay(THROW_DELAY);
          }
          keepAway.next();
        }
      }
    })();
  };

  return (
    <div class={styles.App}>
      <Show when={wasmLoaded()}>
        <Switch>
          <Match when={showInput()}>
            <Input input={input} setInput={setInput} onStart={onStart} />
          </Match>
          <Match when={!showInput()}>
            <RoundInfo round={model.round} />

            <Catcher flipped={model.catcher_flipped} />

            <For each={model.monkies}>{(monkey, i) =>
              <Monkey pos={monkey.pos} itemsInspected={monkey.itemsInspected} />
            }</For>

            <For each={model.items}>{(item, i) =>
              <Item pos={item.pos} index={item.index} worryLevel={item.worryLevel} />
            }</For>
          </Match>
        </Switch>
      </Show>
    </div>
  );
};

const RoundInfo: Component<{ round: number }> = (props) => {
  return (
    <div class={styles.RoundInfo}>Round {props.round}</div>
  );
}

const Catcher: Component<{ flipped: boolean }> = (props) => {
  return (
    <img
      src={catcher}
      class={styles.Catcher}
      style={{
        transform: `scaleX(${ props.flipped ? -1 : 1 })`,
      }}
    />
  );
}

const Monkey: Component<{ pos: number, itemsInspected: number }> = (props) => {
  return (
    <>
      <img
        src={monkey}
        class={styles.Monkey}
        style={{
          transform: `scaleX(${ props.pos < 0.25 || props.pos > 0.75 ? -1 : 1 })`,
          top: `${25 * Math.sin(props.pos * 2 * Math.PI)}vh`,
          left: `${25 * Math.cos(props.pos * 2 * Math.PI)}vh`,
        }}
      />
      <div
        class={styles.MonkeyInfo}
        style={{
          top: `${25 * Math.sin(props.pos * 2 * Math.PI)}vh`,
          left: `${25 * Math.cos(props.pos * 2 * Math.PI)}vh`,
        }}
        >{props.itemsInspected}</div>
    </>
  );
}

const Item: Component<{ pos: number, index: number, worryLevel: number }> = (props) => {
  return (
    <div
      class={styles.Item}
      style={{
        translate: `${ props.index === 0 ? (props.pos < 0.25 || props.pos > 0.75 ? "-8vh" : "-1vh") : "-50% -50%" } 1vh`,
        left: `${((props.index === 0 ? 0 : 5) + 25 + 6 * props.index) * Math.cos(props.pos * 2 * Math.PI)}vh`,
        top: `${((props.index === 0 ? 0 : 6) + 25 + 3 * props.index) * Math.sin(props.pos * 2 * Math.PI)}vh`,
        "z-index": 100 - props.index,
        transition: `all ${THROW_DELAY / 1000}s ease-out`
      }}
    >
      {props.worryLevel}
    </div>
  );
}

const Input: Component<{ input: any, setInput: any, onStart: () => void }> = (props) => {
  return (
    <div class={styles.Input}>
      <textarea
        class={styles.InputTextArea}
        value={props.input()}
        onInput={(e) => props.setInput(e.currentTarget.value)} />
      <div class={styles.InputFooter}>
        <button class={styles.InputStartButton} onClick={(e) => props.onStart()}>Start</button>
      </div>
    </div>
  );
}


export default App;
