import type { Type704 } from '../batch-03/file-0704.js';
export interface Type705 {
  id: 705;
  name: 'File705';
  next: Type704;
}

export function make705(): Type705 {
  return { id: 705, name: 'File705', next: null as unknown as Type704 };
}
