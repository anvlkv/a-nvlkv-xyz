let rvHandles = {};
let visible = null;
let active = null;

const observer = new ResizeObserver(() => {
  Object.values(rvHandles).forEach(({ r }) => r.resizeDrawingSurfaceToCanvas());
});

export function cleanUp(artboard) {
  let { [`${artboard}`]: handle, ...rvHandlesRest } = rvHandles;

  if (handle) {
    handle.r.cleanup();
    observer.unobserve(handle.el);
  }
  rvHandles = rvHandlesRest;
}

export function mountArtboard(artboard) {
  const stateMachines = `${artboard} State Machine`;
  const handle = {};
  handle.el = document.getElementById(`stepper_icon_${artboard}`);
  handle.r = new window.rive.Rive({
    src: "/pkg/anvlkv-done-button.riv",
    canvas: handle.el,
    autoplay: true,
    stateMachines,
    artboard,
    layout: new window.rive.Layout({
      fit: "cover",
      alignment: "center",
    }),
    onLoad: () => {
      handle.r.resizeDrawingSurfaceToCanvas();
      handle.inactiveInput = handle.r
        .stateMachineInputs(stateMachines)
        .find((input) => input.name === "Inactive");
      handle.visibleInput = handle.r
        .stateMachineInputs(stateMachines)
        .find((input) => input.name === "Visible");
      handle.inactiveInput.value = active != artboard;
      handle.visibleInput.value = visible == artboard;
      observer.observe(handle.el);
    },
  });
  rvHandles[artboard] = handle;
}

export function setActive(artboard) {
  active = artboard;
  Object.entries(rvHandles).forEach(([key, { inactiveInput }]) => {
    if (inactiveInput) {
      inactiveInput.value = key != artboard;
    }
  });
}

export function setVisible(artboard) {
  visible = artboard;
  Object.entries(rvHandles).forEach(([key, { visibleInput }]) => {
    if (visibleInput) {
      visibleInput.value = key == artboard;
    }
  });
}

export function forgetVisible() {
  visible = null;
  Object.values(rvHandles).forEach(({ visibleInput }) => {
    if (visibleInput) {
      visibleInput.value = false;
    }
  });
}
