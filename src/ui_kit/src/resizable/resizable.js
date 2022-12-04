; (() => {
  const container = document.currentScript.parentElement
  const content = container.firstElementChild
  const handle = container.querySelector(".resize-handle")

  const isVertical = container.classList.contains("vertical")

  let isDragging = false
  let initialEvent = null
  let initialClientRect = null

  handle.addEventListener("mousedown", (event) => {
    addEventListeners()
    isDragging = true
    initialEvent = event
    initialClientRect = container.getBoundingClientRect()
  })

  function handleMouseMove(event) {
    if (!isDragging || !initialEvent || !initialClientRect) {
      return
    }

    const delta = isVertical
      ? event.y - initialEvent.y
      : event.x - initialEvent.x
    const value = isVertical
      ? initialClientRect.height + delta
      : initialClientRect.width + delta

    content.style[isVertical ? "height" : "width"] = `${value}px`
  }
  function handleMouseUp() {
    removeEventListeners()
    isDragging = false
    initialEvent = null
  }
  function addEventListeners() {
    document.addEventListener("mousemove", handleMouseMove)
    document.addEventListener("mouseup", handleMouseUp)
  }
  function removeEventListeners() {
    if (!isDragging) {
      return
    }
    document.removeEventListener("mousemove", handleMouseMove)
    document.removeEventListener("mouseup", handleMouseUp)
  }
})()
