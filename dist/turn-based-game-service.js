export class TurnBasedGameService {
    constructor(cellClient, zomeName) {
        this.cellClient = cellClient;
        this.zomeName = zomeName;
    }
    /** These functions **must** match the functions defined in lib/src/mixin.rs */
    getGameMoves(gameHash) {
        return this.callZome('get_game_moves', gameHash);
    }
    getMyCurrentGames() {
        return this.callZome('get_my_current_games', null);
    }
    getGame(gameHash) {
        return this.callZome('get_game', gameHash);
    }
    callZome(fnName, payload) {
        return this.cellClient.callZome(this.zomeName, fnName, payload);
    }
}
//# sourceMappingURL=turn-based-game-service.js.map