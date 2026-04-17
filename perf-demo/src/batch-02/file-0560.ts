import type { Type559 } from '../batch-02/file-0559.js';
export interface Type560 {
  id: 560;
  name: 'File560';
  next: Type559;
}

export function make560(): Type560 {
  return { id: 560, name: 'File560', next: null as unknown as Type559 };
}
