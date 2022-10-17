self.onmessage = (e) => {
  let counter = 0
  setInterval(() => {
    counter++

    if (counter === 1000) {
      counter = 0
    }
    self.postMessage(counter)
  }, 1000)
}

export{};