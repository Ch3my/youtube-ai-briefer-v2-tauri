import { invoke } from "@tauri-apps/api/core";

async function getYoutubeMediaInfo(url: string) {
    const mediaId = extractVideoId(url)
    if(!mediaId){
        throw new Error(`No se pudo obtener ID del Video: ${url}`);
    }
	try {
		const searchUrl = await buildSearchUrl(mediaId);
		const response = await fetch(searchUrl);
		if (!response.ok) {
			throw new Error(`YouTube API Error: ${response.status}`);
		}

		const data = await response.json();
		return extractMediaInfo(data);
	} catch (error: any) {
		console.error(`Error: ${error.message}`);
		return null;
	}
}

/**
 * Builds the search URL for the YouTube API request.
 * @param {string} mediaId - The YouTube media ID.
 * @param {string} mediaType - The type of the media.
 * @return {string} The search URL for the YouTube API.
 */
async function buildSearchUrl(mediaId: string) {
    const apiKey = await invoke("get_env", {name: "YOUTUBE_API_KEY"});
	return `https://www.googleapis.com/youtube/v3/videos?part=snippet,statistics,contentDetails&id=${mediaId}&key=${apiKey}`;
}

/**
 * Extracts detailed information from the YouTube API response.
 * @param {object} data - The API response data.
 * @param {string} mediaType - The type of the media.
 * @return {object|null} The extracted media information or null if extraction fails.
 */
function extractMediaInfo(data: any) {
	if (!data.items || data.items.length === 0) return null;

	const info = data.items[0].snippet;
	const thumbnails = info.thumbnails;

	const result = {
		thumbnailUrl: thumbnails["standard"]?.url,
		description: info?.description, 
        title: "",
        channelTitle: ""
	};

    result.title = info?.title;
    result.channelTitle = info?.channelTitle;

	return result;
}

function extractVideoId(url: string): string {
    const regex = /[?&]v=([^&#]+)/; // Regular expression to capture the video ID
    const match = url.match(regex);  // Match the URL against the regex

    return match ? match[1] : "";   // Return the video ID if found, else null
}

export { getYoutubeMediaInfo };