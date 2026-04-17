import type { Type720 } from '../batch-03/file-0720.js';
export interface Type721 {
  id: 721;
  name: 'File721';
  next: Type720;
}

export function make721(): Type721 {
  return { id: 721, name: 'File721', next: null as unknown as Type720 };
}
