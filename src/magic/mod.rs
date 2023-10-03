use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_hanabi::prelude::*;

pub struct MagicPlugin;

impl Plugin for MagicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, particle_effect);
    }
}

fn particle_effect(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(0., 1., 0., 1.));
    gradient.add_key(1.0, Vec4::new(0., 1., 0., 0.));

    let writer = ExprWriter::new();

    let age = writer.lit(0.).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    let lifetime = writer.lit(5.).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let init_pos = SetPositionCircleModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        axis: writer.lit(Vec3::Z).expr(),
        radius: writer.lit(10.).expr(),
        dimension: ShapeDimension::Volume,
    };

    let init_vel = SetVelocityCircleModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        axis: writer.lit(Vec3::Z).expr(),
        speed: writer.lit(10.).expr(),
    };

    let spawner = Spawner::rate(30.0.into());
    println!("{:?}", spawner);
    println!("{:?}", spawner.is_once());
    let effect = effects.add(
        EffectAsset::new(4096, spawner, writer.finish())
            .with_name("2d")
            .init(init_pos)
            .init(init_vel)
            .init(init_age)
            .init(init_lifetime)
            .render(SizeOverLifetimeModifier {
                gradient: Gradient::constant(Vec2::splat(20.)),
                screen_space_size: false,
            })
            .render(ColorOverLifetimeModifier { gradient }),
    );

    commands
        .spawn(ParticleEffectBundle {
            effect: ParticleEffect::new(effect),
            transform: Transform::from_translation(Vec3::new(-100., -100., 10.)),
            ..default()
        })
        .insert(Name::new("emitter"))
        .with_children(|p| {
            p.spawn(MaterialMesh2dBundle {
                mesh: meshes
                    .add(bevy::prelude::shape::Quad::new(Vec2::splat(10.)).into())
                    .into(),
                material: materials.add(ColorMaterial::from(Color::PINK)),
                transform: Transform::from_translation(Vec3::splat(0.)),
                ..default()
            });
        });
}
