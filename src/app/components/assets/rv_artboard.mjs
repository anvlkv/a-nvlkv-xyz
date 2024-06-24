const fetched = {};

const loadRiveFile = async (src) => {
  const cached = fetched[src];
  if (cached) {
    return cached.slice(0);
  } else {
    const req = new Request(src);
    const res = await fetch(req);
    const buffer = await res.arrayBuffer();
    fetched[src] = buffer.slice(0);
    return buffer;
  }
};

/**
 *     Binding to RiveJs
 *      @class RvJS
 */
export class RvJs {
  /**
   *     @property{*} inputs
   */
  inputs = {};
  /**
   *     @property{*} triggers
   */
  triggers = [];
  /**
   *     @property{Rive | null} rv
   */
  rv = null;
  /**
   *     @property{string | null} stateMachine
   */
  stateMachine = null;
  /**
   *     @property{Element | null} el
   */
  el = null;

  observer = new ResizeObserver(() => {
    if (this.rv) {
      this.rv.resizeDrawingSurfaceToCanvas();
    }
  });

  /**
   *     Mount Rive animation
   *
   *     @this{RvJs} this
   *     @param{string} file
   *     @param{string} name
   *     @param{string} stateMachine
   *     @param{Element} el
   *     @param{string} fit
   *     @param{string} alignment
   *     @return{Promise<void>}
   */

  async mountAnimation(file, name, stateMachine, el, fit, alignment) {
    const buffer = await loadRiveFile(file);
    await new Promise((resolve) => {
      this.rv = new window.rive.Rive({
        buffer,
        canvas: el,
        autoplay: true,
        stateMachines: stateMachine,
        artboard: name,
        artboard: name,
        layout: new window.rive.Layout({
          fit,
          alignment,
        }),
        onLoad: () => {
          this.rv.resizeDrawingSurfaceToCanvas();
          this.observer.observe(el);
          Object.entries(this.inputs).forEach(([name, { value }]) => {
            const input = this.rv
              .stateMachineInputs(stateMachine)
              .find((input) => input.name === name);
            input.value = value;
            this.inputs[name] = input;
          });
          this.triggers.forEach((name, i) => {
            const input = this.rv
              .stateMachineInputs(stateMachine)
              .find((input) => input.name === name);
            input.fire();
            this.triggers[i] = input;
          });
          this.stateMachine = stateMachine;
          resolve();
        },
      });
    });
  }

  /**
   *     Unmount Rive animation
   *
   *     @this{RvJs} thiis
   *     @return{void}
   */
  cleanUp() {
    if (this.rv) {
      this.rv.cleanup();
    }
    this.rv = null;
    this.observer.disconnect();
    this.el = null;
    this.stateMachine = null;
    this.triggers = [];
    this.inputs = {};
  }

  /**
   *     Set input value
   *
   *     @this{RvJs} this
   *     @param{string} name
   *     @param{any} value
   *     @return{void}
   */
  setInput(name, value) {
    if (this.inputs[name] && this.inputs[name].name == name) {
      this.inputs[name].value = value;
    } else if (this.rv && this.stateMachine) {
      const input = this.rv
        .stateMachineInputs(this.stateMachine)
        .find((input) => input.name === name);
      this.inputs[name] = input;
      input.value = value;
    } else {
      this.inputs[name] = { value };
    }
  }

  /**
   *     Trigger input
   *
   *     @this{RvJs} this
   *     @param{string} name
   *     @return{void}
   */
  trigerInput(name) {
    const trigger = this.triggers.find((t) => t.name == name);
    if (trigger) {
      trigger.fire();
    } else if (this.rv && this.stateMachine) {
      const input = this.rv
        .stateMachineInputs(this.stateMachine)
        .find((input) => input.name === name);
      this.triggers.push(input);
      input.fire();
    } else if (!this.triggers.includes(name)) {
      this.triggers.push(name);
    }
  }
}
