import type { Type9 } from '../batch-00/file-0009.js';
export interface Type10 {
  id: 10;
  name: 'File10';
  next: Type9;
}

export function make10(): Type10 {
  return { id: 10, name: 'File10', next: null as unknown as Type9 };
}
