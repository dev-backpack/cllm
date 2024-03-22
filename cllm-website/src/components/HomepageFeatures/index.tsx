import clsx from 'clsx';
import Heading from '@theme/Heading';
import styles from './styles.module.css';

type FeatureItem = {
  title: string;
  Svg: React.ComponentType<React.ComponentProps<'svg'>>;
  description: JSX.Element;
};

type ExampleItem = {
  url: string;
  description: JSX.Element;
};

const ExampleList: ExampleItem[] = [
  {
    // gif image of the example
    url: 'img/cllm-example-1.gif',
    description: (
      <>
        <p>
          <strong>Example 1</strong>: This is an example of a feature that is
          available in CLLM.
        </p>
      </>
    ),
  }
];


function Feature({title, Svg, description}: FeatureItem) {
  return (
    <div className={clsx('col col--4')}>
      <div className="text--center">
        <Svg className={styles.featureSvg} role="img" />
      </div>
      <div className="text--center padding-horiz--md">
        <Heading as="h3">{title}</Heading>
        <p>{description}</p>
      </div>
    </div>
  );
}

function Example({url, description}: ExampleItem) {
  return (
    <div className={clsx('col')}>
      <div className="text--center">
        <img src={url} />
      </div>
      <div className="text--center padding-horiz--md">
        <p>{description}</p>
      </div>
    </div>
  );
}

export default function HomepageFeatures(): JSX.Element {
  return (
    <section className={styles.features}>
      <div className="container">
        <div className="row">
          {ExampleList.map((props, idx) => (
            <Example key={idx} {...props} />
          ))}
          {/* {FeatureList.map((props, idx) => (
            <Feature key={idx} {...props} />
          ))} */}
        </div>
      </div>
    </section>
  );
}
