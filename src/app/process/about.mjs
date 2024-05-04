const artboards = ["Problem", "Solution", "Compromise", "Implement", "Iterate"];

let rv = null;

export function cleanUp() {
  if (rv) {
    rv.forEach((r) => r.cleanup());
    rv = null;
  }
}

export function mountArtboards() {
  rv = artboards.map((artboard) => {
    const r = new window.rive.Rive({
      src: "/pkg/anvlkv-done-button.riv",
      canvas: document.getElementById(`about_icon_${artboard}`),
      autoplay: true,
      artboard,
      layout: new window.rive.Layout({
        fit: "fit",
        alignment: "center",
      }),
      onLoad: () => {
        r.resizeDrawingSurfaceToCanvas();
      },
    });
    return r;
  });
}
