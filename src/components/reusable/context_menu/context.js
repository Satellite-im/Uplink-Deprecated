document.getElementById("ID").addEventListener(
  "contextmenu",
  function (ev) {
    ev.stopPropagation()
    ev.preventDefault()
    const context_menu = document.getElementById("context-menu")
    context_menu.classList.remove("hidden")
    if (context_menu.offsetWidth + ev.pageX > document.offsetWidth) {
      context_menu.style.right = `${ev.pageX}px`
    } else {
      context_menu.style.left = `${ev.pageX}px`
    }
    if (context_menu.offsetHeight + ev.pageY > document.offsetHeight) {
      context_menu.style.bottom = `${ev.pageX}px`
    } else {
      context_menu.style.top = `${ev.pageX}px`
    }

    return false
  },
  false,
)

document.addEventListener("click", (ev) => {
  const context_menu = document.getElementById("context-menu")
  context_menu.classList.add("hidden")
})
