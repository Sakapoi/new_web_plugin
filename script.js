export function getActiveTabUrl(callback) {
  chrome.tabs.query({}, tabs => {
    let urls = tabs.map(obj => obj.url);
    callback(urls);
});
}

export function changeContent(content) {
  // Знаходимо div за id
  const contentDiv = document.getElementById('content');
  
  // Змінюємо innerHTML div
  contentDiv.innerHTML = content;
}