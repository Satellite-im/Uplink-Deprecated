var folder_name_p_element = document.getElementById("folder_id-name-normal")
var folder_name_input_element = document.getElementById("folder_id-input")
var input_value = ""

folder_name_input_element.addEventListener("input", (_) => {
  input_value = folder_name_input_element.value
})

var show_input_and_focus = function (_) {
  folder_name_p_element.style.display = "none"
  folder_name_input_element.style.display = "block"
  folder_name_input_element.value = input_value
  folder_name_input_element.focus()
}

show_input_and_focus()

document.addEventListener("click", (event) => {
  if (folder_name_input_element.style.display === "block") {
    show_input_and_focus()
  }

  var file_element = document.getElementById("folder_id-folder")
  if (!file_element.contains(event.target)) {
    folder_name_input_element.value = ""
    folder_name_input_element.style.display = "none"
    folder_name_p_element.style.display = "block"
    removeEventListener("click")
  }
})
