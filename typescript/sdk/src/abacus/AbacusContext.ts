import { BigNumberish, ethers } from 'ethers';
import { MultiProvider } from '..';
import { Inbox } from '@abacus-network/core';
import { BridgeToken, BridgeToken__factory } from '@abacus-network/apps';
import { BridgeContracts } from './contracts/BridgeContracts';
import { Governor, CoreContracts } from './contracts/CoreContracts';
import { ResolvedTokenInfo, TokenIdentifier } from './tokens';
import { canonizeId, evmId } from '../utils';
import {
  devDomains,
  mainnetDomains,
  mainnetLegacyDomains,
  AbacusDomain,
  testnetDomains,
  testnetLegacyDomains,
} from './domains';
import { TransferMessage } from './messages';
import { hexlify } from '@ethersproject/bytes';

type Address = string;

/**
 * The AbacusContext managers connections to Abacus core and Bridge contracts.
 * It inherits from the {@link MultiProvider}, and ensures that its contracts
 * always use the latest registered providers and signers.
 *
 * For convenience, we've pre-constructed contexts for mainnet and testnet
 * deployments. These can be imported directly.
 *
 * @example
 * // Set up mainnet and then access contracts as below:
 * let router = mainnet.mustGetBridge('celo').bridgeRouter;
 */
export class AbacusContext extends MultiProvider {
  private cores: Map<number, CoreContracts>;
  private bridges: Map<number, BridgeContracts>;

  constructor(
    domains: AbacusDomain[],
    cores: CoreContracts[],
    bridges: BridgeContracts[],
  ) {
    super();
    domains.forEach((domain) => this.registerDomain(domain));
    this.cores = new Map();
    this.bridges = new Map();

    cores.forEach((core) => {
      this.cores.set(core.domain, core);
    });
    bridges.forEach((bridge) => {
      this.bridges.set(bridge.domain, bridge);
    });
  }

  /**
   * Instantiate an AbacusContext from contract info.
   *
   * @param domains An array of Domains with attached contract info
   * @returns A context object
   */
  static fromDomains(domains: AbacusDomain[]): AbacusContext {
    const cores = domains.map((domain) => CoreContracts.fromObject(domain));
    const bridges = domains.map((domain) => BridgeContracts.fromObject(domain));
    return new AbacusContext(domains, cores, bridges);
  }

  /**
   * Ensure that the contracts on a given domain are connected to the
   * currently-registered signer or provider.
   *
   * @param domain the domain to reconnect
   */
  private reconnect(domain: number) {
    const connection = this.getConnection(domain);
    if (!connection) {
      throw new Error(`Reconnect failed: no connection for ${domain}`);
    }
    // re-register contracts
    const core = this.cores.get(domain);
    if (core) {
      core.connect(connection);
    }
    const bridge = this.bridges.get(domain);
    if (bridge) {
      bridge.connect(connection);
    }
  }

  /**
   * Register an ethers Provider for a specified domain.
   *
   * @param nameOrDomain A domain name or number.
   * @param provider An ethers Provider to be used by requests to that domain.
   */
  registerProvider(
    nameOrDomain: string | number,
    provider: ethers.providers.Provider,
  ): void {
    const domain = this.resolveDomain(nameOrDomain);
    super.registerProvider(domain, provider);
    this.reconnect(domain);
  }

  /**
   * Register an ethers Signer for a specified domain.
   *
   * @param nameOrDomain A domain name or number.
   * @param signer An ethers Signer to be used by requests to that domain.
   */
  registerSigner(nameOrDomain: string | number, signer: ethers.Signer): void {
    const domain = this.resolveDomain(nameOrDomain);
    super.registerSigner(domain, signer);
    this.reconnect(domain);
  }

  /**
   * Remove the registered ethers Signer from a domain. This function will
   * attempt to preserve any Provider that was previously connected to this
   * domain.
   *
   * @param nameOrDomain A domain name or number.
   */
  unregisterSigner(nameOrDomain: string | number): void {
    const domain = this.resolveDomain(nameOrDomain);
    super.unregisterSigner(domain);
    this.reconnect(domain);
  }

  /**
   * Clear all signers from all registered domains.
   */
  clearSigners(): void {
    super.clearSigners();
    this.domainNumbers.forEach((domain) => this.reconnect(domain));
  }

  /**
   * Get the {@link CoreContracts} for a given domain (or undefined)
   *
   * @param nameOrDomain A domain name or number.
   * @returns a {@link CoreContracts} object (or undefined)
   */
  getCore(nameOrDomain: string | number): CoreContracts | undefined {
    const domain = this.resolveDomain(nameOrDomain);
    return this.cores.get(domain);
  }

  /**
   * Get the {@link CoreContracts} for a given domain (or throw an error)
   *
   * @param nameOrDomain A domain name or number.
   * @returns a {@link CoreContracts} object
   * @throws if no {@link CoreContracts} object exists on that domain.
   */
  mustGetCore(nameOrDomain: string | number): CoreContracts {
    const core = this.getCore(nameOrDomain);
    if (!core) {
      throw new Error(`Missing core for domain: ${nameOrDomain}`);
    }
    return core;
  }

  /**
   * Get the {@link BridgeContracts} for a given domain (or undefined)
   *
   * @param nameOrDomain A domain name or number.
   * @returns a {@link BridgeContracts} object (or undefined)
   */
  getBridge(nameOrDomain: string | number): BridgeContracts | undefined {
    const domain = this.resolveDomain(nameOrDomain);
    return this.bridges.get(domain);
  }
  /**
   * Get the {@link BridgeContracts} for a given domain (or throw an error)
   *
   * @param nameOrDomain A domain name or number.
   * @returns a {@link BridgeContracts} object
   * @throws if no {@link BridgeContracts} object exists on that domain.
   */
  mustGetBridge(nameOrDomain: string | number): BridgeContracts {
    const bridge = this.getBridge(nameOrDomain);
    if (!bridge) {
      throw new Error(`Missing bridge for domain: ${nameOrDomain}`);
    }
    return bridge;
  }

  /**
   * Resolve the inbox for the Outbox domain on the Remote domain (if any).
   *
   * WARNING: do not hold references to this contract, as it will not be
   * reconnected in the event the chain connection changes.
   *
   * @param outbox the sending domain
   * @param remote the receiving domain
   * @returns An interface for the Inbox (if any)
   */
  getInboxFor(
    outbox: string | number,
    remote: string | number,
  ): Inbox | undefined {
    return this.getCore(remote)?.getInbox(this.resolveDomain(outbox));
  }

  /**
   * Resolve the inbox for the Outbox domain on the Remote domain (or throws).
   *
   * WARNING: do not hold references to this contract, as it will not be
   * reconnected in the event the chain connection changes.
   *
   * @param outbox the sending domain
   * @param remote the receiving domain
   * @returns An interface for the Inbox
   * @throws If no inbox is found.
   */
  mustGetInboxFor(outbox: string | number, remote: string | number): Inbox {
    const inbox = this.getInboxFor(outbox, remote);
    if (!inbox) {
      throw new Error(`Missing inbox for outbox ${outbox} & remote ${remote}`);
    }
    return inbox;
  }

  /**
   * Returns the governors of this abacus deployment.
   *
   * @returns The governors of the deployment
   */
  async governors(): Promise<Governor[]> {
    let governors = await Promise.all(
      Array.from(this.cores.values()).map((core) => core.governor()),
    );
    governors = governors.filter(
      (governor) => governor.identifier !== ethers.constants.AddressZero,
    );
    if (governors.length === 0) throw new Error('no governors');
    return governors;
  }

  /**
   * Returns the single governor of this deployment, throws an error if not found.
   *
   * @returns The governor of the deployment
   */
  async governor(): Promise<Governor> {
    const governors = await this.governors();
    if (governors.length !== 1) throw new Error('multiple governors');
    return governors[0];
  }

  /**
   * Discovers the governor domain of this abacus deployment and returns the
   * associated Core.
   *
   * @returns The identifier of the governing domain
   */
  async governorCore(): Promise<CoreContracts> {
    const governor = await this.governor();
    return this.mustGetCore(governor.domain);
  }

  /**
   * Resolve the local representation of a token on some domain. E.g. find the
   * deployed Celo address of Ethereum's Sushi Token.
   *
   * WARNING: do not hold references to this contract, as it will not be
   * reconnected in the event the chain connection changes.
   *
   * @param nameOrDomain the target domain, which hosts the representation
   * @param token The token to locate on that domain
   * @returns An interface for that token (if it has been deployed on that
   * domain)
   */
  async resolveRepresentation(
    nameOrDomain: string | number,
    token: TokenIdentifier,
  ): Promise<BridgeToken | undefined> {
    const domain = this.resolveDomain(nameOrDomain);
    const bridge = this.getBridge(domain);

    const tokenDomain = this.resolveDomain(token.domain);
    const tokenId = canonizeId(token.id);

    const address = await bridge?.bridgeRouter[
      'getLocalAddress(uint32,bytes32)'
    ](tokenDomain, tokenId);

    if (!address) {
      return;
    }

    let contract = new BridgeToken__factory().attach(evmId(address));

    const connection = this.getConnection(domain);
    if (connection) {
      contract = contract.connect(connection);
    }
    return contract;
  }

  /**
   * Resolve the local representation of a token on ALL known domain. E.g.
   * find ALL deployed addresses of Ethereum's Sushi Token, on all registered
   * domains.
   *
   * WARNING: do not hold references to these contracts, as they will not be
   * reconnected in the event the chain connection changes.
   *
   * @param token The token to locate on ALL domains
   * @returns A {@link ResolvedTokenInfo} object with representation addresses
   */
  async resolveRepresentations(
    token: TokenIdentifier,
  ): Promise<ResolvedTokenInfo> {
    const tokens: Map<number, BridgeToken> = new Map();

    await Promise.all(
      this.domainNumbers.map(async (domain) => {
        const tok = await this.resolveRepresentation(domain, token);
        if (tok) {
          tokens.set(domain, tok);
        }
      }),
    );

    return {
      domain: this.resolveDomain(token.domain),
      id: token.id,
      tokens,
    };
  }

  /**
   * Resolve the canonical domain and identifier for a representation on some
   * domain.
   *
   * @param nameOrDomain The domain hosting the representation
   * @param representation The address of the representation on that domain
   * @returns The domain and ID for the canonical token
   * @throws If the token is unknown to the bridge router on its domain.
   */
  async resolveCanonicalIdentifier(
    nameOrDomain: string | number,
    representation: Address,
  ): Promise<TokenIdentifier> {
    const domain = this.resolveDomain(nameOrDomain);
    const bridge = this.mustGetBridge(nameOrDomain);
    const repr = hexlify(canonizeId(representation));

    const canonical = await bridge.bridgeRouter.representationToCanonical(
      representation,
    );

    if (canonical[0] !== 0) {
      return {
        domain: canonical[0],
        id: canonical[1],
      };
    }

    // check if it's a local token
    const local = await bridge.bridgeRouter['getLocalAddress(uint32,bytes32)'](
      domain,
      repr,
    );
    if (local !== ethers.constants.AddressZero) {
      return {
        domain,
        id: hexlify(canonizeId(local)),
      };
    }

    // throw
    throw new Error('Token not known to the bridge');
  }

  /**
   * Resolve an interface for the canonical token corresponding to a
   * representation on some domain.
   *
   * @param nameOrDomain The domain hosting the representation
   * @param representation The address of the representation on that domain
   * @returns An interface for that token
   * @throws If the token is unknown to the bridge router on its domain.
   */
  async resolveCanonicalToken(
    nameOrDomain: string | number,
    representation: Address,
  ): Promise<BridgeToken> {
    const canonicalId = await this.resolveCanonicalIdentifier(
      nameOrDomain,
      representation,
    );
    if (!canonicalId) {
      throw new Error('Token seems to not exist');
    }
    const token = await this.resolveRepresentation(
      canonicalId.domain,
      canonicalId,
    );
    if (!token) {
      throw new Error(
        'Cannot resolve canonical on its own domain. how did this happen?',
      );
    }
    return token;
  }

  /**
   * Send tokens from one domain to another. Approves the bridge if necessary.
   *
   * @param from The domain to send from
   * @param to The domain to send to
   * @param token The token to send
   * @param amount The amount (in smallest unit) to send
   * @param recipient The identifier to send to on the `to` domain
   * @param overrides Any tx overrides (e.g. gas price)
   * @returns a {@link TransferMessage} object representing the in-flight
   *          transfer
   * @throws On missing signers, missing tokens, tx issues, etc.
   */
  async send(
    from: string | number,
    to: string | number,
    token: TokenIdentifier,
    amount: BigNumberish,
    recipient: Address,
    overrides: ethers.Overrides = {},
  ): Promise<TransferMessage> {
    const fromBridge = this.mustGetBridge(from);
    const bridgeAddress = fromBridge.bridgeRouter.address;

    const fromToken = await this.resolveRepresentation(from, token);
    if (!fromToken) {
      throw new Error(`Token not available on ${from}`);
    }
    const sender = this.getSigner(from);
    if (!sender) {
      throw new Error(`No signer for ${from}`);
    }
    const senderAddress = await sender.getAddress();

    const approved = await fromToken.allowance(senderAddress, bridgeAddress);
    // Approve if necessary
    if (approved.lt(amount)) {
      const tx = await fromToken.approve(bridgeAddress, amount, overrides);
      await tx.wait();
    }

    const tx = await fromBridge.bridgeRouter.send(
      fromToken.address,
      amount,
      this.resolveDomain(to),
      canonizeId(recipient),
      overrides,
    );
    const receipt = await tx.wait();

    const message = TransferMessage.singleFromReceipt(this, from, receipt);
    if (!message) {
      throw new Error();
    }

    return message as TransferMessage;
  }

  /**
   * Send a chain's native asset from one chain to another using the
   * `EthHelper` contract.
   *
   * @param from The domain to send from
   * @param to The domain to send to
   * @param amount The amount (in smallest unit) to send
   * @param recipient The identifier to send to on the `to` domain
   * @param overrides Any tx overrides (e.g. gas price)
   * @returns a {@link TransferMessage} object representing the in-flight
   *          transfer
   * @throws On missing signers, tx issues, etc.
   */
  async sendNative(
    from: string | number,
    to: string | number,
    amount: BigNumberish,
    recipient: Address,
    overrides: ethers.PayableOverrides = {},
  ): Promise<TransferMessage> {
    const ethHelper = this.mustGetBridge(from).ethHelper;
    if (!ethHelper) {
      throw new Error(`No ethHelper for ${from}`);
    }

    const toDomain = this.resolveDomain(to);

    overrides.value = amount;

    const tx = await ethHelper.sendToEVMLike(toDomain, recipient, overrides);
    const receipt = await tx.wait();

    const message = TransferMessage.singleFromReceipt(this, from, receipt);
    if (!message) {
      throw new Error();
    }

    return message as TransferMessage;
  }
}

export const testnetLegacy = AbacusContext.fromDomains(testnetLegacyDomains);
export const mainnetLegacy = AbacusContext.fromDomains(mainnetLegacyDomains);
export const dev = AbacusContext.fromDomains(devDomains);
export const testnet = AbacusContext.fromDomains(testnetDomains);
export const mainnet = AbacusContext.fromDomains(mainnetDomains);