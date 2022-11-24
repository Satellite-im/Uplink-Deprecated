var media = document.getElementById("media-content")

function setMediaSize(width, height) {
  ;[...document.getElementsByClassName("media-user")].forEach((el) => {
    el.style.width = `${width}px`
    el.style.height = `${height}px`
  })
}

function calculateMediaUserSize(entry) {
  if (!media) return
  const [width, height] = getOptimalBoxDimensions({
    containerWidth: entry?.contentRect.width ?? media.clientWidth,
    containerHeight: entry?.contentRect.height ?? media.clientHeight,
    aspectRatio: 16 / 9,
    numBoxes: document.getElementsByClassName("media-user").length,
    gap: 16,
  })

  setMediaSize(width, height)
}

function getOptimalBoxDimensions(params) {
  let prevWidth = 0
  let prevHeight = 0
  let width = 0
  let height = 0
  for (let numRows = 1; numRows <= params.numBoxes; numRows++) {
    const numMaxCols = Math.ceil(params.numBoxes / numRows)
    prevWidth = width
    prevHeight = height
    ;[width, height] = getBoxDimensionsForLayout(params, numMaxCols, numRows)
    if (prevWidth > width) {
      return [prevWidth, prevHeight]
    }
  }
  return [width, height]
}

function getBoxDimensionsForLayout(params, numMaxCols, numRows) {
  const rowGap = params.gap * (numRows - 1)
  const colGap = params.gap * (numMaxCols - 1)
  let boxWidth = (params.containerWidth - colGap) / numMaxCols
  let boxHeight = boxWidth / params.aspectRatio
  const contentHeight = boxHeight * numRows + rowGap
  if (contentHeight > params.containerHeight) {
    boxHeight = (params.containerHeight - rowGap) / numRows
    boxWidth = boxHeight * params.aspectRatio
  }
  return [boxWidth, boxHeight]
}

var resizeObserver = new ResizeObserver((entries) => {
  entries.forEach((entry) => {
    calculateMediaUserSize(entry)
  })
})

if (media) resizeObserver.observe(media)
