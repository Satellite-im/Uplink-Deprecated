var file_name_p_element = document.getElementById("file_id-name-normal")
var file_name_input_element = document.getElementById("file_id-input")
file_name_p_element.style.display = "none"
file_name_input_element.style.display = "block"
file_name_input_element.focus()

document.addEventListener("click", (event) => {
  var file_element = document.getElementById("file_id-file")
  if (!file_element.contains(event.target)) {
    file_name_input_element.style.display = "none"
    file_name_p_element.style.display = "block"
  }
})
