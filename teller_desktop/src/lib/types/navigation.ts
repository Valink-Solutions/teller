export interface Directory {
	[name: string]: string | string[] | null;
}

export interface CurrentDir {
	type?: string;
	category: string | null;
	path: string;
}
