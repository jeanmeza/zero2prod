-- Add migration script here
begin;
  -- Backfill `status` for historical entries
  update subscriptions
    set status = 'confirmed'
    where status is null;
  -- Make `status` mandatory
  alter table subscriptions alter column status set not null;
commit;
  
