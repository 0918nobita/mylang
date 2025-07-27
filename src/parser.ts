export const parse = (source: string) => {
    const segmenter = new Intl.Segmenter();

    const segments = Array.from(segmenter.segment(source));

    console.log(segments.map((segment) => segment.segment));
};
