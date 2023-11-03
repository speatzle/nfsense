# Metacomponents
This folder contains components which don't represent specific visual elements, but rather annotate other components to add specific functionality.
Due to their more confusing nature, their function and implementation is described here.

## Portal
This component wraps Vues standard `Teleport` component, providing semantic meaning to a `Teleport` target and not just the source.

It also fixes an issue where one could not update the `Teleport` source by tracking tracking active targets and not rendering the source when there is no active target for that key.

## PageHeader
This component uses the above `Portal` imlpementation to allow updating the Page Header elsewhere in the layout, whilst also setting the title and providing a basic consistent layout.
