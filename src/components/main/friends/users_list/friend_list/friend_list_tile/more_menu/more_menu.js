document.addEventListener("click", (event) => {
  //close the more menu when clicking outside of it and outside of more button
  if (
    !more_menu.contains(event.target) &&
    !more_button.contains(event.target)
  ) {
    more_menu.style.display = "none"
    removeEventListener("click")
  }
})
