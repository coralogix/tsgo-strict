import type { Type200 } from '../batch-01/file-0200.js';
export interface Type201 {
  id: 201;
  name: 'File201';
  next: Type200;
}

export function make201(): Type201 {
  return { id: 201, name: 'File201', next: null as unknown as Type200 };
}
