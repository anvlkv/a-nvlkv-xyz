const mainState = "Sun-Moon State Machine";
const artboard = "Sun-Moon";

let rv = null;
let isDark = false;
let isDarkInput = null;

const observer = new ResizeObserver(() => {
  if (rv) {
    rv.resizeDrawingSurfaceToCanvas();
  }
});

export function cleanUp() {
  if (rv) {
    rv.cleanup();
  }
  rv = null;
  observer.disconnect();
  isDarkInput = null;
}

export function darkModeAnimation(isDarkModeEnabled) {
  isDark = isDarkModeEnabled;

  const layout = new window.rive.Layout({
    fit: "cover",
    alignment: "center",
  });
  const el = document.getElementById("dark-mode_animation");
  rv = new window.rive.Rive({
    src: "/pkg/anvlkv-done-button.riv",
    canvas: el,
    autoplay: true,
    stateMachines: mainState,
    artboard,
    layout,
    onLoad: () => {
      rv.resizeDrawingSurfaceToCanvas();
      observer.observe(el);
      isDarkInput = rv
        .stateMachineInputs(mainState)
        .find((input) => input.name === "IsDark");
      isDarkInput.value = isDark;
    },
  });
}

export function setDark(isDarkModeEnabled) {
  isDark = isDarkModeEnabled;

  if (isDarkInput) {
    isDarkInput.value = isDarkModeEnabled;
  }
}
