export function getActiveTabUrl(callback) {
  chrome.tabs.query({}, tabs => {
    let urls = tabs.map(obj => obj.url);
    callback(urls);
  });
}

export function changeContent(content) {
  const contentDiv = document.getElementById('content');

  contentDiv.innerHTML = content;
}

export function listProcesses() {
  console.log("peter");
  chrome.processes.getProcessInfo(function(processes) {
    for (var i = 0; i < processes.length; i++) {
      console.log(processes[i]);
    }
  });
}

export async function takeScreenshott() {
  const stream = await navigator.mediaDevices.getDisplayMedia({ video: true });
  const track = stream.getVideoTracks()[0];
  const imageCapture = new ImageCapture(track);
  const blob = await imageCapture.takePhoto();
  const url = URL.createObjectURL(blob);
  const img = new Image();
  img.src = url;
  document.getElementById("screenshot").appendChild(img);
}

export const capture = async () => {
  const canvas = document.createElement("canvas");
  const context = canvas.getContext("2d");
  const video = document.createElement("video");

  try {
    const captureStream = await navigator.mediaDevices.getDisplayMedia();
    video.srcObject = captureStream;
    context.drawImage(video, 0, 0, window.width, window.height);
    console.log("1");
    const frame = canvas.toDataURL("image/png");
    console.log("2");
    captureStream.getTracks().forEach(track => track.stop());
    window.location.href = frame;
  } catch (err) {
    console.error("Error: " + err);
  }
};

export function screen() {
  chrome.tabs.create({ url: "https://www.youtube.com/watch?v=dQw4w9WgXcQ" }, function(tab) {
    // Виконуємо скріпт на сторінці
    chrome.tabs.executeScript(tab.id, { takeScreenshotAndCopyToClipboard });
  });
}

export function takeScreenshotAndDownload() {
  // Створюємо новий елемент canvas
  const canvas = document.createElement('canvas');
  
  // Встановлюємо розміри canvas такими, які має екран
  canvas.width = window.innerWidth;
  canvas.height = window.innerHeight;
  
  // Отримуємо контекст для малювання на canvas
  const ctx = canvas.getContext('2d');
  
  // Заповнюємо canvas зображенням екрану
  ctx.drawImage(document.body, 0, 0, canvas.width, canvas.height);
  
  // Створюємо посилання на завантаження зображення
  const link = document.createElement('a');
  link.download = 'screenshot.png';
  
  // Перетворюємо canvas в URL-адрес зображення
  canvas.toBlob(blob => {
    const url = URL.createObjectURL(blob);
    link.href = url;
    
    // Клікаємо по посиланню, щоб завантажити зображення
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
    
    // Звільняємо URL-адрес зображення
    URL.revokeObjectURL(url);
  }, 'image/png');
}

export function captureAndCopyToClipboard() {
  // Створюємо новий об'єкт Image для зберігання скріншоту
  var img = new Image();

  // Викликаємо функцію для створення скріншоту та передаємо callback, щоб коли скріншот буде готовий, ми могли скопіювати його до буфера обміну
  chrome.tabs.captureVisibleTab(null, { format: "png" }, function(dataUrl) {
    img.src = dataUrl;

    // Створюємо новий об'єкт canvas для роботи зі скріншотом
    var canvas = document.createElement("canvas");
    canvas.width = img.width;
    canvas.height = img.height;

    // Рендеримо скріншот на canvas
    var context = canvas.getContext("2d");
    context.drawImage(img, 0, 0);

    // Отримуємо URL зображення з canvas
    var imageUrl = canvas.toDataURL("image/png");
    //console.log(imageUrl);
    // Копіюємо зображення до буфера обміну
    var imgData = atob(imageUrl.substring("data:image/png;base64,".length));
    var buffer = new ArrayBuffer(imgData.length);
    var view = new Uint8Array(buffer);
    for (var i = 0; i < imgData.length; i++) {
      view[i] = imgData.charCodeAt(i);
    }
    var blob = new Blob([buffer], { type: "image/png" });
    var data = [new ClipboardItem({ "image/png": blob })];
    navigator.clipboard.write(data).then(function() {
      console.log("Скріншот був скопійований до буфера обміну!");
    }, function() {
      console.error("Помилка копіювання скріншоту до буфера обміну!");
    });
  });
}

export function qwe() {
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
