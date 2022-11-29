var media = document.querySelector("#media-content")

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

function calculateMediaUserSize(entry) {
  const mediaUsers = media.querySelectorAll(".media-user")
  const dimensions = getOptimalBoxDimensions({
    containerWidth: entry?.contentRect.width ?? media.clientWidth,
    containerHeight: entry?.contentRect.height ?? media.clientHeight,
    aspectRatio: 16 / 9,
    numBoxes: mediaUsers.length,
    gap: 16,
  })

  for (const mediaUser of mediaUsers) {
    mediaUser.style.width = `${dimensions[0]}px`
    mediaUser.style.height = `${dimensions[1]}px`
  }
}

var resizeObserver = new ResizeObserver((entries) => {
  for (const entry of entries) {
    calculateMediaUserSize(entry)
  }
})

if (media) resizeObserver.observe(media)
