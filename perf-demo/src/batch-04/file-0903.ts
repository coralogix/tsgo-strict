import type { Type902 } from '../batch-04/file-0902.js';
export interface Type903 {
  id: 903;
  name: 'File903';
  next: Type902;
}

export function make903(): Type903 {
  return { id: 903, name: 'File903', next: null as unknown as Type902 };
}
