import type { Type203 } from '../batch-01/file-0203.js';
export interface Type204 {
  id: 204;
  name: 'File204';
  next: Type203;
}

export function make204(): Type204 {
  return { id: 204, name: 'File204', next: null as unknown as Type203 };
}
