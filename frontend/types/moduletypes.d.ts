declare module "*.svg" {
    const content: any;
    export default content;
}

declare module "$stores/local" {
    export const token: any;
} 

declare function goto(path:string);

declare module "timeago-simple" {
    const timeago: {
        simple: (date: string | Date) => string,
        future: (date: string | Date) => string
    }
    export default timeago
}