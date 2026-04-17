import type { Type625 } from '../batch-03/file-0625.js';
export interface Type626 {
  id: 626;
  name: 'File626';
  next: Type625;
}

export function make626(): Type626 {
  return { id: 626, name: 'File626', next: null as unknown as Type625 };
}
