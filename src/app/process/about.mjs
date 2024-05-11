const artboards = [
  "Problem",
  "Solution",
  "Compromise",
  "Implement",
  "Iterate",
  "Privacy",
];

let rv = null;

const observer = new ResizeObserver(() => {
  if (rv) {
    rv.forEach((r) => r.resizeDrawingSurfaceToCanvas());
  }
});

export function cleanUp() {
  if (rv) {
    rv.forEach((r) => r.cleanup());
    rv = null;
    observer.disconnect();
  }
}

export function mountArtboards() {
  rv = artboards.map((artboard) => {
    const el = document.getElementById(`about_icon_${artboard}`);
    const r = new window.rive.Rive({
      src: "/pkg/anvlkv-done-button.riv",
      canvas: el,
      autoplay: true,
      artboard,
      layout: new window.rive.Layout({
        fit: "contain",
        alignment: "center",
      }),
      onLoad: () => {
        r.resizeDrawingSurfaceToCanvas();
        observer.observe(el);
      },
    });
    return r;
  });
}
