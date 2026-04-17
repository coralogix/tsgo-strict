import type { Type408 } from '../batch-02/file-0408.js';
export interface Type409 {
  id: 409;
  name: 'File409';
  next: Type408;
}

export function make409(): Type409 {
  return { id: 409, name: 'File409', next: null as unknown as Type408 };
}
