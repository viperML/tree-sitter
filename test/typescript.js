export function formatDate(date) {
    return date.toLocaleDateString("en-uk", {
        year: "numeric",
        month: "long",
        day: "numeric",
    })
}
