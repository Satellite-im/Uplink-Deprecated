var container = document.getElementById("scroll-messages-container")
var isLockedToBottom = true

function debounce(func, timeout = 100) {
  let timer
  return (...args) => {
    clearTimeout(timer)
    timer = setTimeout(() => {
      func.apply(this, args)
    }, timeout)
  }
}

container.addEventListener("scroll", debounce(onScroll))

function onScroll() {
  isLockedToBottom =
    container.scrollTop + container.clientHeight >= container.scrollHeight - 1
}

function scrollToBottom() {
  if (!container) return

  container.scrollTop = container.scrollHeight
}

var messages = document.getElementById("scroll-messages")
var config = { childList: true }

function callback(mutationList) {
  for (const mutation of mutationList) {
    if (mutation.type === "childList") {
      const node = mutation.addedNodes[0]
      const isSameAuthor = node?.dataset?.remote === "false"

      if (isSameAuthor) {
        scrollToBottom()
      }
    }
  }
}

var resizeObserver = new ResizeObserver(() => {
  if (isLockedToBottom) {
    scrollToBottom()
  }
})
if (container) resizeObserver.observe(container)

var observer = new MutationObserver(callback)
if (messages) observer.observe(messages, config)
