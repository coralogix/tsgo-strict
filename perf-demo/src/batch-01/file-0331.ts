import type { Type330 } from '../batch-01/file-0330.js';
export interface Type331 {
  id: 331;
  name: 'File331';
  next: Type330;
}

export function make331(): Type331 {
  return { id: 331, name: 'File331', next: null as unknown as Type330 };
}
