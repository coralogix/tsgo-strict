import type { Type299 } from '../batch-01/file-0299.js';
export interface Type300 {
  id: 300;
  name: 'File300';
  next: Type299;
}

export function make300(): Type300 {
  return { id: 300, name: 'File300', next: null as unknown as Type299 };
}
