var file_name_p_element = document.getElementById("file_id-name-normal")
var file_name_input_element = document.getElementById("file_id-input")
var input_value = ""

file_name_input_element.addEventListener("input", (_) => {
  input_value = file_name_input_element.value
})

var show_input_and_focus = function (_) {
  file_name_p_element.style.display = "none"
  file_name_input_element.style.display = "block"
  file_name_input_element.value = input_value
  file_name_input_element.focus()
}

show_input_and_focus()

document.addEventListener("click", (event) => {
  if (file_name_input_element.style.display === "block") {
    show_input_and_focus()
  }

  var file_element = document.getElementById("file_id-file")
  if (!file_element.contains(event.target)) {
    file_name_input_element.value = ""
    file_name_input_element.style.display = "none"
    file_name_p_element.style.display = "block"
    removeEventListener("click")
  }
})
