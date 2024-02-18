export default async function postData(url = '', data = {}) {
    const response = await fetch('/api' + url, {
        method: 'POST',
        cache: 'no-cache',
        referrerPolicy: 'no-referrer',
        body: JSON.stringify(data)
    });
    return response;
}