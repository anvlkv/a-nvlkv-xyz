const artboards = [
  "About",
  "Problem",
  "Solution",
  "Compromise",
  "Implement",
  "Iterate",
  "Inquire",
];

let rvHandles = null;
let visible = null;
let active = null;

export function cleanUp() {
  if (rvHandles) {
    rvHandles.forEach(({ r }) => r.cleanup());
    rvHandles = null;
  }
}

export function mountArtboards() {
  rvHandles = artboards.map((artboard) => {
    const stateMachines = `${artboard} State Machine`;
    const handle = {};
    handle.r = new window.rive.Rive({
      src: "/pkg/anvlkv-done-button.riv",
      canvas: document.getElementById(`stepper_icon_${artboard}`),
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
      },
    });
    return handle;
  });
}

export function setActive(artboard) {
  active = artboard;
  const i = artboards.indexOf(artboard);
  rvHandles.forEach(({ inactiveInput }, ii) => {
    if (inactiveInput) {
      inactiveInput.value = i != ii;
    }
  });
}

export function setVisible(artboard) {
  visible = artboard;
  const i = artboards.indexOf(artboard);
  rvHandles.forEach(({ visibleInput }, ii) => {
    if (visibleInput) {
      visibleInput.value = i == ii;
    }
  });
}

export function forgetVisible() {
  visible = null;
  rvHandles.forEach(({ visibleInput }, ii) => {
    if (visibleInput) {
      visibleInput.value = false;
    }
  });
}
