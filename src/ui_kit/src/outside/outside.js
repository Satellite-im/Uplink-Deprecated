function hideOnClickOutside(element, trigger) {
  const outsideClickListener = (event) => {
    if (
      !element.contains(event.target) &&
      !trigger.contains(event.target) &&
      isVisible(element)
    ) {
      element.style.display = "none"
      trigger.style.backgroundColor = "var(--theme-secondary)"
      removeClickListener()
    }
  }

  const removeClickListener = () => {
    document.removeEventListener("click", outsideClickListener)
  }

  document.addEventListener("click", outsideClickListener)
}

function isVisible(elem) {
  return (
    !!elem &&
    !!(elem.offsetWidth || elem.offsetHeight || elem.getClientRects().length)
  )
}

function initClick() {
  const container = document.getElementById("outside-container-ID")
  if (container.children.length < 2) {
    throw new Error(
      "Outside click needs a container (the hiding container) as first child and a trigger button as second child",
    )
  }

  const el = container.firstChild
  el.style.display = "none"

  const trigger = container.children[1]
  trigger.style.backgroundColor = "var(--theme-secondary)"

  trigger.addEventListener("click", () => {
    el.style.display = el.style.display === "none" ? "" : "none"
    trigger.style.backgroundColor =
      el.style.display === "none"
        ? "var(--theme-secondary)"
        : "var(--theme-primary)"

    hideOnClickOutside(el, trigger)
  })
}

initClick()
