import {Column, Entity, PrimaryColumn} from 'typeorm';
@Entity()
class Settle{
    @PrimaryColumn()
    id:string;
    @Column()
    type:string;
    @Column()
    created:number;
    @Column()
    channel:string
    @Column("simple-json")
    recipient: {}
    @Column()
    wallet_id:string
    @Column("simple-json")
    extra:{};
}

export default Settle
