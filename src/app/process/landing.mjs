const mainState = "Done button stage State Machine";
const artboard = "Done button stage";

let rv = null;

export function cleanUp() {
  if (rv) {
    rv.cleanup();
    rv = null;
  }
}

export function landingAnimation() {
  const layout = new window.rive.Layout({
    fit: "cover",
    alignment: "bottomCenter",
  });

  rv = new window.rive.Rive({
    src: "/pkg/anvlkv-done-button.riv",
    canvas: document.getElementById("process_animation"),
    autoplay: true,
    stateMachines: mainState,
    artboard,
    layout,
    onLoad: () => {
      rv.resizeDrawingSurfaceToCanvas();

      const showInput = rv
        .stateMachineInputs(mainState)
        .find((input) => input.name === "Show");

      const button = document.getElementById("the-done-button");

      button.addEventListener("pointerenter", () => {
        showInput.value = true;
      });
      button.addEventListener("pointerleave", () => {
        showInput.value = false;
      });
    },
  });
}
