const mainState = "Privacy State Machine";
const artboard = "Privacy";

let rv = null;

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

export function privacyAnimation() {
  const layout = new window.rive.Layout({
    fit: "cover",
    alignment: "center",
  });
  const el = document.getElementById("privacy_animation");
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
    },
  });
}
