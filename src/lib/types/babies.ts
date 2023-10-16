export type Baby = {
	name: string;
	sex: string;
	photo?: string;
	birthday: string;
	created_at: string;
};

export type Babies = Array<Baby>;
