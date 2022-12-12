import { Component, createSignal, For, Match, Show, Switch } from 'solid-js';
import { createStore, produce } from 'solid-js/store';
import init, { KeepAway, example } from 'wasm';

import catcher from './assets/catcher.png';
import monkey from './assets/monkey.png';
import styles from './App.module.css';

type Model = {
  round: number,
  catcher_flipped: boolean,
  monkies: Monkey[],
  items: Item[],
  monkeyBusiness: number,
};

type Monkey = {
  pos: number,
  items: number[],
  itemsInspected: number,
  ranking: number,
};

type Item = {
  pos: number,
  index: number,
  worryLevel: number
};

const delay = (d: number) => d === 0 ? 0 : new Promise(resolve => setTimeout(resolve, d));

const App: Component = () => {
  const [wasmLoaded, setWasmLoaded] = createSignal(false);
  const [showInput, setShowInput] = createSignal(true);
  const [input, setInput] = createSignal("");
  const [speed, setSpeed] = createSignal(1);
  const [model, setModel] = createStore<Model>({
    round: 1,
    catcher_flipped: false,
    monkies: [],
    items: [],
    monkeyBusiness: 0,
  });

  let keepAway = undefined as unknown as KeepAway;
  init().then(() => {
    setWasmLoaded(true);
    setInput(example());
  });

  const DELAY = () => Math.pow(3 - speed(), 3) * 10;

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
        itemsInspected: 0,
        ranking: 0,
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
              for (let i = 0; i < draft.monkies[monkeyIndex].items.length; i++) {
                draft.items[draft.monkies[monkeyIndex].items[i]].index = i;
              }
            }));
            await delay(DELAY());

            setModel(produce(draft => {
              draft.items[model.monkies[monkeyIndex].items[0]].worryLevel = keepAway.inspect();
              draft.monkies[monkeyIndex].itemsInspected++;
            }));
            await delay(0.5 * DELAY());

            setModel(produce(draft => draft.items[draft.monkies[monkeyIndex].items[0]].worryLevel = keepAway.get_borred()));
            await delay(0.5 * DELAY());

            let throwTo = 0;
            let index = 0;
            setModel(produce(draft => {
              throwTo = keepAway.throw();
              index = draft.monkies[monkeyIndex].items.splice(0, 1)[0];
              draft.monkies[throwTo].items.push(index);
              draft.items[index].pos = draft.monkies[throwTo].pos;
              draft.items[index].index = 0;
            }));
            await delay(DELAY());

            setModel(produce(draft => {
              draft.items[index].index = draft.monkies[throwTo].items.length;
            }));
            await delay(DELAY());
          }
          keepAway.next();
        }
      }
      await delay(1000);
      setModel(produce(draft => {
        const rankings = draft.monkies.map((monkey, index) => [index, monkey.itemsInspected]);
        rankings.sort((a, b) => b[1] - a[1]).forEach(([index, itemsInspected], ranking) => draft.monkies[index].ranking = ranking + 1);
      }));
      await delay(1000);
      setModel(produce(draft => {
        draft.monkeyBusiness = keepAway.monkey_business();
      }));
    })();
  };

  return (
    <div class={styles.App}>
      <Show when={wasmLoaded()}>
        <Switch>
          <Match when={showInput()}>
            <Input input={input} setInput={setInput} speed={speed} setSpeed={setSpeed} onStart={onStart} />
          </Match>
          <Match when={!showInput()}>
            <RoundInfo round={model.round} />

            <Catcher flipped={model.catcher_flipped} />

            <For each={model.monkies}>{(monkey, i) =>
              <>
                <Monkey pos={monkey.pos} />
                <MonkeyInfo pos={monkey.pos} itemsInspected={monkey.itemsInspected} ranking={monkey.ranking} />
              </>
            }</For>

            <MonkeyInfo pos={0} itemsInspected={model.monkeyBusiness} ranking={-1} />
            <MonkeyBusiness monkeyBusiness={model.monkeyBusiness} />

            <For each={model.items}>{(item, i) =>
              <Item pos={item.pos} index={item.index} worryLevel={item.worryLevel} delay={DELAY()} />
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

const Monkey: Component<{ pos: number }> = (props) => {
  return (
    <img
      src={monkey}
      class={styles.Monkey}
      style={{
        transform: `scaleX(${ props.pos < 0.25 || props.pos > 0.75 ? -1 : 1 })`,
        top: `${25 * Math.sin(props.pos * 2 * Math.PI)}vh`,
        left: `${25 * Math.cos(props.pos * 2 * Math.PI)}vh`,
      }}
    />
  );
}

const MonkeyInfo: Component<{ pos: number, itemsInspected: number, ranking: number }> = (props) => {
  return (
    <div
      class={styles.MonkeyInfo}
      classList={{
        [styles.MonkeyInfoBusy]: props.ranking === 0,
        [styles.MonkeyInfoFirst]: props.ranking === 1,
        [styles.MonkeyInfoSecond]: props.ranking === 2,
        [styles.MonkeyInfoHidden]: props.ranking > 2,
        [styles.MonkeyInfoBusinessHidden]: props.ranking === -1 && props.itemsInspected === 0,
        [styles.MonkeyInfoBusiness]: props.ranking === -1 && props.itemsInspected > 0,
      }}
      style={{
        "--top": `${25 * Math.sin(props.pos * 2 * Math.PI)}vh`,
        "--left": `${25 * Math.cos(props.pos * 2 * Math.PI)}vh`,
      }}
      >{props.itemsInspected}</div>
  );
}

const Item: Component<{ pos: number, index: number, worryLevel: number, delay: number }> = (props) => {
  return (
    <div
      class={styles.Item}
      style={{
        translate: `${ props.index === 0 ? (props.pos < 0.25 || props.pos > 0.75 ? "-8vh" : "-1vh") : "-50% -50%" } 1vh`,
        left: `${((props.index === 0 ? 0 : 5) + 25 + 6 * props.index) * Math.cos(props.pos * 2 * Math.PI)}vh`,
        top: `${((props.index === 0 ? 0 : 6) + 25 + 3 * props.index) * Math.sin(props.pos * 2 * Math.PI)}vh`,
        "z-index": 100 - props.index,
        transition: `all ${props.delay / 1000}s ease-out`
      }}
    >
      {props.worryLevel}
    </div>
  );
}

const Input: Component<{ input: any, setInput: any, speed: any, setSpeed: any, onStart: () => void }> = (props) => {
  const speedText = () => ["Slow", "Normal", "Fast", "The Flash"][props.speed()];

  return (
    <div class={styles.Input}>
      <textarea
        class={styles.InputTextArea}
        value={props.input()}
        onInput={(e) => props.setInput(e.currentTarget.value)} />
      <div class={styles.InputFooter}>
        <span class={styles.InputRangeSpan}>
          Speed:
          <input type="range" class={styles.InputRangeSlider} min={0} max={3} value={props.speed()} onInput={(e) => props.setSpeed(e.currentTarget.value)} />
          {speedText()}
        </span>
        <button class={styles.InputStartButton} onClick={(e) => props.onStart()}>Start</button>
      </div>
    </div>
  );
}

const MonkeyBusiness: Component<{ monkeyBusiness: number }> = (props) => {
  const style1 = () => ({ opacity: props.monkeyBusiness === 0 ? 0 : 0.8 });
  const style2 = () => ({ opacity: props.monkeyBusiness === 0 ? 0 : 1 });
  return (
    <>
      <div class={styles.MonkeyBusinessStrip} style={style1()} />
      <div class={styles.MonkeyBusinessTimes} style={style2()}>&times;</div>
      <div class={styles.MonkeyBusinessEquals} style={style2()}> = </div>
    </>
  );
}

export default App;
