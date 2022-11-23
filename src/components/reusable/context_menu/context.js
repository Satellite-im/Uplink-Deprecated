document.getElementById("ID").addEventListener(
  "contextmenu",
  function (ev) {
    ev.stopPropagation()
    ev.preventDefault()
    const context_menu = document.getElementById("context-menu")
    context_menu.classList.remove("hidden")
    context_menu.style.top = `${ev.pageY}px`
    context_menu.style.left = `${ev.pageX}px`
    return false
  },
  false,
)

document.addEventListener("click", (ev) => {
  const context_menu = document.getElementById("context-menu")
  context_menu.classList.add("hidden")
})
