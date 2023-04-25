export function getActiveTabUrl(callback) {
  chrome.tabs.query({}, tabs => {
    let urls = tabs.map(obj => obj.url);
    callback(urls);
  });
}

export function openLinkInNewTab(url) {
  document.getElementById("anime").addEventListener("click", function(event) {
    event.preventDefault(); // зупинка дії за замовчуванням
    window.open(url, '_blank');
  });
}

export function changeContent(content, id) {
  const contentDiv = document.getElementById(id);
  contentDiv.innerHTML = content;
}

export function listProcesses() {
  chrome.processes.getProcessInfo(function(processes) {
    for (var i = 0; i < processes.length; i++) {
      console.log(processes[i]);
    }
  });
}

export function screen() {
  chrome.tabs.create({ url: "https://www.youtube.com/watch?v=dQw4w9WgXcQ" }, function(tab) {
    // Виконуємо скріпт на сторінці
    chrome.tabs.executeScript(tab.id, { takeScreenshotAndCopyToClipboard });
  });
}

export function take_screen_from_js() {
  // Request media
  navigator.mediaDevices.getDisplayMedia().then(stream => 
  {
    // Grab frame from stream
    let track = stream.getVideoTracks()[0];
    let capture = new ImageCapture(track);
    capture.grabFrame().then(bitmap => 
    {
      // Stop sharing
      track.stop();
      var canvas = document.getElementById("myCanvas");
      // Draw the bitmap to canvas
      canvas.width = bitmap.width;
      canvas.height = bitmap.height;
      canvas.getContext('2d').drawImage(bitmap, 0, 0);
        
      
      var imageUrl = canvas.toDataURL();
      console.log('url:', imageUrl);
      // var win = window.open("https://example.com/", '_blank');
      // win.focus();
    });
  })
  .catch(e => console.log(e));
}
