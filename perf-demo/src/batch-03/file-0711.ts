import type { Type710 } from '../batch-03/file-0710.js';
export interface Type711 {
  id: 711;
  name: 'File711';
  next: Type710;
}

export function make711(): Type711 {
  return { id: 711, name: 'File711', next: null as unknown as Type710 };
}
