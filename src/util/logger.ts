import log4js from 'log4js';
log4js.configure({
    appenders: {
        console:{ type: 'console' },
        walletLogs:{ type: 'file', filename: 'logs/wallet.log', category: 'wallet' }
    },
    categories: {

        default: {appenders: ['console', 'walletLogs'], level: 'info'}

    }
});
export default log4js.getLogger('wallet');
