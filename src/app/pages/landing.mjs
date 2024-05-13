const mainState = "Done button stage State Machine";
const artboard = "Done button stage";

let rv = null;
let active = false;
let activeInput = null;

const observer = new ResizeObserver(() => {
  if (rv) {
    rv.resizeDrawingSurfaceToCanvas();
  }
});

export function cleanUp() {
  if (rv) {
    rv.cleanup();
    rv = null;
    observer.disconnect();
  }
}

export function landingAnimation() {
  const layout = new window.rive.Layout({
    fit: "fitHeight",
    alignment: "bottomCenter",
  });
  const el = document.getElementById("process_animation");
  return new Promise((resolve) => {
    rv = new window.rive.Rive({
      src: "/pkg/anvlkv-done-button.riv",
      canvas: el,
      autoplay: true,
      stateMachines: mainState,
      artboard,
      layout,
      onLoad: () => {
        rv.resizeDrawingSurfaceToCanvas();

        activeInput = rv
          .stateMachineInputs(mainState)
          .find((input) => input.name === "Show");

        observer.observe(el);

        activeInput.value = active;

        resolve();
      },
    });
  });
}

export function setActive(value) {
  active = value;
  if (activeInput) {
    activeInput.value = value;
  }
}
