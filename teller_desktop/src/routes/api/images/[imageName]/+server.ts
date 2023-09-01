import { promises as fs } from 'fs';

export async function get({ params }) {
	const { imageName } = params;

	try {
		const image = await fs.readFile(`static/images/${imageName}`);
		return {
			headers: {
				'Content-Type': 'image/png' // Set the appropriate content type
			},
			body: image
		};
	} catch (error) {
		return {
			status: 404,
			body: 'Image not found'
		};
	}
}
