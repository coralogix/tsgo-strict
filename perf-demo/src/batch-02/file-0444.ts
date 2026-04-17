import type { Type443 } from '../batch-02/file-0443.js';
export interface Type444 {
  id: 444;
  name: 'File444';
  next: Type443;
}

export function make444(): Type444 {
  return { id: 444, name: 'File444', next: null as unknown as Type443 };
}
