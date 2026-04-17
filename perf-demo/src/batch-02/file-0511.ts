import type { Type510 } from '../batch-02/file-0510.js';
export interface Type511 {
  id: 511;
  name: 'File511';
  next: Type510;
}

export function make511(): Type511 {
  return { id: 511, name: 'File511', next: null as unknown as Type510 };
}
